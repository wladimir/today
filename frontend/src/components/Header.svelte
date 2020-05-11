<script>
    import Icon from 'svelte-awesome';
    import heart from 'svelte-awesome/icons/heart';

    export let handleClick;
</script>

<nav class="text-2xl text-primary bg-primary relative flex flex-wrap items-center justify-between px-2 py-3 navbar-expand-lg mb-3">
    <div class="container px-4 mx-auto flex flex-wrap items-center justify-between">
        <div class="w-full relative flex justify-between lg:w-auto px-4 lg:static lg:block lg:justify-start">
            <a class="font-bold leading-relaxed inline-block mr-4 py-2 whitespace-no-wrap uppercase"
               href="#pablo">
                Today
            </a>
            <button class="cursor-pointer leading-none border border-solid border-transparent rounded bg-transparent block lg:hidden outline-none focus:outline-none"
                    type="button">
                <span class="block relative w-6 h-px rounded-sm bg-white"></span>
                <span class="block relative w-6 h-px rounded-sm bg-white mt-1"></span>
                <span class="block relative w-6 h-px rounded-sm bg-white mt-1"></span>
            </button>
        </div>
        <div class="lg:flex flex-grow items-center">
            <ul class="flex flex-col lg:flex-row list-none ml-auto">
                <li class="nav-item">
                    <a class="px-4 py-4 flex text-sm uppercase font-bold leading-snug hover:opacity-75"
                       href="#sign-in">
                        Sign In
                    </a>
                </li>
                <li class="nav-item">
                    <a class="px-4 py-4 flex text-sm items-center uppercase font-bold leading-snug hover:opacity-75"
                       href="#sign-up">
                        Sign Up
                    </a>
                </li>
                <button
                        class="bg-transparent hover:text-secondary hover:opacity-75 hover:bg-secondary font-semibold py-2 px-4 hover:border-transparent rounded"
                        on:click="{handleClick}">
                    <Icon data="{heart}" />
                </button>
            </ul>
        </div>
    </div>
</nav>
