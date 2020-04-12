<script>
    import { onMount } from 'svelte';

    let text = '';

    const API_BASE_URL = 'http://localhost:3000/';

    onMount(async () => {
        await fetch(API_BASE_URL)
                .then(data => data.json())
                .then(data => text = data['hello']);
    });
</script>

<h1 class="text-yellow">{text}</h1>

<style global>
    h1 {
        @apply bg-blue-200;
    }
</style>
