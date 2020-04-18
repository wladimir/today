<!--suppress CssInvalidAtRule -->
<script>
    import Hello from './Hello.svelte';
    import Login from './Login.svelte';
</script>

<svelte:head>
    <title>Productive Notes</title>
</svelte:head>

<main>
    <Hello />
    <!--    <Login />-->
</main>

<style global>
    @tailwind base;
    @tailwind components;
    @tailwind utilities;
</style>

