<script>
    import Item from './Item.svelte';
    import Icon from 'svelte-awesome';
    import beer from 'svelte-awesome/icons/beer';
    import SortableList from './SortableList.svelte';

    import { onMount } from 'svelte';

    let text = 'abc';

    let list = ['First Item', 'Second Item', 'Third Item'];

    const sortList = ev => {list = ev.detail;};

    const API_BASE_URL = 'http://localhost:3000/';

    // onMount(async () => {
    //     await fetch(API_BASE_URL)
    //             .then(data => data.json())
    //             .then(data => text = data['hello']);
    // });
</script>

<style global>
    h1 {
        @apply bg-blue-200;
    }
</style>

<h1 class="text-yellow">{text}</h1>
<Icon data={beer} />
<div class="list">
    <SortableList
            {list}
            on:sort={sortList}
    />
</div>

