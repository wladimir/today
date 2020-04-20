<script>
    import Badge from './Badge.svelte';
    import Item from './Item.svelte';
    import Icon from 'svelte-awesome';
    import beer from 'svelte-awesome/icons/beer';
    import SortableList from './SortableList.svelte';
    import { THEME } from './Constants';

    import { onMount } from 'svelte';

    let d = document.documentElement;
    let themeStorage;

    let text = 'abc';

    let list = ['First Item', 'Second Item', 'Third Item'];

    const sortList = ev => {list = ev.detail;};

    const API_BASE_URL = 'http://localhost:3000/';

    const handleClick = () => {
        if (d.classList.contains('theme-dark')) {
            d.classList.remove('theme-dark');
            localStorage.removeItem(THEME);
        } else {
            d.classList.add('theme-dark');
            localStorage.setItem(THEME, 'dark');
            themeStorage = 'dark';
        }
    };

    onMount(async () => {
        themeStorage = localStorage.getItem(THEME);

        if (themeStorage === 'dark') {
            d.classList.add('theme-dark');
        }

        //     await fetch(API_BASE_URL)
        //             .then(data => data.json())
        //             .then(data => text = data['hello']);
    });
</script>

<style global>
    div {
        font-family: 'rubik';
    }
</style>

<h1>{text}</h1>
<h2>{JSON.stringify(localStorage)}</h2>
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
<Badge />

