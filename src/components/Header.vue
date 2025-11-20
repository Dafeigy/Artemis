<template>
    <div class="flex w-full h-1/10 dark:bg-[#1d1d1d]/50 dark:text-white bg-[#f9f0f3]/50 items-center justify-between">
        <div id="topleft" class="flex items-center h-full opacity-100">
            <div class="mx-2 rounded-full bg-green-300 h-4/5 aspect-square"></div>
            <input type="text" placeholder="BAUD">
            <input type="text" placeholder="COM">
        </div>
        <div id="topright" class="flex">
            <input type="text" placeholder="Serial Input"></input>
            <div class="flex">
                <button class="bg-green-300 text-white px-4 py-2 rounded-md">Send</button>
            </div>
        </div>
        
    </div>
</template>