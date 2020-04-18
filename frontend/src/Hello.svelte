<script>
    import Item from './Item.svelte';
    import Icon from 'svelte-awesome';
    import beer from 'svelte-awesome/icons/beer';
    import SortableList from './SortableList.svelte';

    import { onMount } from 'svelte';

    let d = document.documentElement;
    let themeStorage = localStorage.getItem('theme');
    if (themeStorage === 'dark') {
        d.classList.add('dark-theme');
    }

    let text = 'abc';

    let list = ['First Item', 'Second Item', 'Third Item'];

    const sortList = ev => {list = ev.detail;};

    const API_BASE_URL = 'http://localhost:3000/';

    const handleClick = () => {
        if (d.classList.contains('theme-dark')) {
            d.classList.remove('theme-dark');
            localStorage.removeItem('theme');
        } else {
            d.classList.add('theme-dark');
            localStorage.setItem('theme', 'dark');
        }
    };

    // onMount(async () => {
    //     await fetch(API_BASE_URL)
    //             .then(data => data.json())
    //             .then(data => text = data['hello']);
    // });
</script>

<style global>
    div {
        font-family: 'rubik';
    }
</style>

<h1>{text}</h1>
<Icon data={beer} />
<div class="font-bold text-4xl text-primary">
    <SortableList
            {list}
            on:sort={sortList}
    />
</div>
<button class="theme-btn font-body rounded-btn text-xl font-medium bg-primary hover:bg-secondary text-primary hover:text-secondary px-6 py-3"
        on:click={handleClick}>Change theme
</button>

