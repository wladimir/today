<!--suppress CssInvalidAtRule -->
<script>
    import Hello from './Hello.svelte';
</script>

<svelte:head>
    <title>Rustappoint</title>
</svelte:head>

<main>
    <Hello />
</main>

<style global>
    @tailwind base;
    @tailwind components;
    @tailwind utilities;
</style>

