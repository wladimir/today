<script>
    import Icon from 'svelte-awesome';
    import adjust from 'svelte-awesome/icons/adjust';

    export let handleClick;
</script>

<nav>
    <div class="bg-primary text-secondary font-extrabold">
        TODAY
        <button class="bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded"
                on:click={handleClick}>
            <Icon data={adjust}/>
        </button>
    </div>
</nav>
