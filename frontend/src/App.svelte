<!--suppress CssInvalidAtRule -->
<script>
    import Hello from './Hello.svelte';
    import Login from './Login.svelte';
    import Header from './Header.svelte';
    import MainView from './MainView.svelte';
</script>

<svelte:head>
    <title>Today</title>
</svelte:head>

<main>
    <Header />
    <MainView />
    <Hello />
    <!--    <Login />-->
</main>

<style global>
    @tailwind base;
    @tailwind components;
    @tailwind utilities;
</style>

