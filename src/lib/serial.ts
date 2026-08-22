import { computed, reactive, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'

export interface PortInfo {
  name: string
  port_type: string
}

interface SerialStatus {
  port_name: string
  baud_rate: number
  is_open: boolean
}

interface SerialTextEvent {
  port_name: string
  data: string
}

interface SerialBytesEvent {
  port_name: string
  data: number[]
}

export const baudRates = [9600, 19200, 38400, 57600, 115200, 230400, 460800, 921600]
export const availablePorts = ref<PortInfo[]>([])
export const selectedPortName = ref('')
export const portBaudRates = reactive<Record<string, number>>({})
export const openPortNames = reactive(new Set<string>())
export const portLogs = reactive<Record<string, string[]>>({})

const terminalBuffers = new Map<string, number[]>()
const terminalSubscribers = new Set<(event: SerialBytesEvent) => void>()
let initialized = false
let refreshTimer: number | undefined
let unlisteners: UnlistenFn[] = []

const ensurePortState = (portName: string) => {
  if (!portBaudRates[portName]) portBaudRates[portName] = 115200
  if (!portLogs[portName]) portLogs[portName] = []
}

const appendLog = (portName: string, message: string) => {
  ensurePortState(portName)
  portLogs[portName]!.push(message)
  if (portLogs[portName]!.length > 5000) portLogs[portName]!.splice(0, 500)
}

const appendTerminalBytes = (event: SerialBytesEvent) => {
  const existing = terminalBuffers.get(event.port_name) ?? []
  existing.push(...event.data)
  if (existing.length > 262_144) existing.splice(0, existing.length - 262_144)
  terminalBuffers.set(event.port_name, existing)
  terminalSubscribers.forEach((subscriber) => subscriber(event))
}

export const getTerminalBuffer = (portName: string) =>
  new Uint8Array(terminalBuffers.get(portName) ?? [])

export const subscribeToTerminal = (subscriber: (event: SerialBytesEvent) => void) => {
  terminalSubscribers.add(subscriber)
  return () => terminalSubscribers.delete(subscriber)
}

export const clearPortLogs = (portName: string) => {
  portLogs[portName]?.splice(0)
}

export const clearTerminalBuffer = (portName: string) => {
  terminalBuffers.delete(portName)
}

export const refreshAvailablePorts = async () => {
  try {
    const ports = await invoke<PortInfo[]>('get_available_ports')
    availablePorts.value = ports
    ports.forEach((port) => ensurePortState(port.name))
    if (!selectedPortName.value && ports[0]) selectedPortName.value = ports[0].name
  } catch (error) {
    console.error('Failed to scan serial ports:', error)
  }
}

const refreshStatuses = async () => {
  try {
    const statuses = await invoke<SerialStatus[]>('get_serial_port_status')
    openPortNames.clear()
    statuses.forEach((status) => {
      if (status.is_open) openPortNames.add(status.port_name)
      portBaudRates[status.port_name] = status.baud_rate
    })
  } catch (error) {
    console.error('Failed to read serial status:', error)
  }
}

export const openPort = async (portName: string) => {
  ensurePortState(portName)
  await invoke('open_serial_port', {
    portName,
    baudRate: portBaudRates[portName],
  })
  openPortNames.add(portName)
}

export const closePort = async (portName: string) => {
  await invoke('close_serial_port', { portName })
  openPortNames.delete(portName)
}

export const togglePort = async (portName: string) => {
  if (openPortNames.has(portName)) await closePort(portName)
  else await openPort(portName)
}

export const sendToPort = (portName: string, data: number[]) =>
  invoke<number>('send_to_serial_port', { portName, data })

export const initializeSerialWorkspace = async () => {
  if (initialized) return
  initialized = true
  if (!('__TAURI_INTERNALS__' in window)) return

  unlisteners = await Promise.all([
    listen<SerialBytesEvent>('serial_data_bytes', ({ payload }) => appendTerminalBytes(payload)),
    listen<SerialTextEvent>('serial_data', ({ payload }) => appendLog(payload.port_name, payload.data)),
    listen<SerialTextEvent>('serial_message', ({ payload }) => appendLog(payload.port_name, payload.data)),
    listen<SerialTextEvent>('serial_error', ({ payload }) => {
      appendLog(payload.port_name, `Error: ${payload.data}`)
      openPortNames.delete(payload.port_name)
    }),
    listen<SerialStatus>('serial_status_changed', ({ payload }) => {
      ensurePortState(payload.port_name)
      portBaudRates[payload.port_name] = payload.baud_rate
      if (payload.is_open) openPortNames.add(payload.port_name)
      else openPortNames.delete(payload.port_name)
    }),
  ])

  await Promise.all([refreshAvailablePorts(), refreshStatuses()])
  refreshTimer = window.setInterval(refreshAvailablePorts, 2500)
}

export const disposeSerialWorkspace = () => {
  if (refreshTimer !== undefined) window.clearInterval(refreshTimer)
  refreshTimer = undefined
  unlisteners.forEach((unlisten) => unlisten())
  unlisteners = []
  initialized = false
}

export const isSerialPortOpen = computed({
  get: () => openPortNames.has(selectedPortName.value),
  set: (isOpen: boolean) => {
    if (!selectedPortName.value) return
    if (isOpen) openPortNames.add(selectedPortName.value)
    else openPortNames.delete(selectedPortName.value)
  },
})
