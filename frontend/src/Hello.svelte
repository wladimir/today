<script>
    import Item from './Item.svelte';
    import Icon from 'svelte-awesome';
    import beer from 'svelte-awesome/icons/beer';

    import { onMount } from 'svelte';

    let text = 'abc';

    let list = [
        { id: 1, name: 'First Item' },
        { id: 2, name: 'Second Item' },
        { id: 3, name: 'Third Item' }
    ];

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
<div>

</div>

