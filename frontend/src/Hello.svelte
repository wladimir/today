<script>
    import { onMount } from 'svelte';

    let text = '';

    onMount(async () => {
        await fetch(`http://localhost:3000/`)
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
