<script>
    import Switch from "./Switch.svelte";
    import Badge from "./Badge.svelte";
    import Icon from "svelte-awesome";
    import beer from "svelte-awesome/icons/beer";
    import SortableList from "./SortableList.svelte";

    let uno = false;

    $: values = { uno };

    import { onMount } from "svelte";

    let text = "abc";

    let list = ["First Item", "Second Item", "Third Item"];

    const sortList = ev => {list = ev.detail;};

    const API_BASE_URL = "http://localhost:3000/";

    onMount(async () => {
        //     await fetch(API_BASE_URL)
        //             .then(data => data.json())
        //             .then(data => text = data['hello']);
    });
</script>

<h1>{text}</h1>
<Icon data={beer} />
<div class="font-bold text-4xl text-primary">
    <SortableList
            {list}
            on:sort={sortList}
    />
</div>
<Badge />
<Switch bind:checked="{uno}" id="uno" text="uno" />
