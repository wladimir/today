<script>
    import Icon from 'svelte-awesome';
    import heart from 'svelte-awesome/icons/heart';

    export let handleClick;
</script>

<nav>
    <div class="bg-primary text-primary font-extrabold text-4xl">
        TODAY
        <button class="bg-transparent hover:bg-blue-500 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded"
                on:click={handleClick}>
            <Icon data={heart}/>
        </button>
        <div class="bg-secondary text-secondary">YO THEME</div>
    </div>
</nav>
