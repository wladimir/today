<!--suppress CssInvalidAtRule -->
<script>
    import Hello from "./Hello.svelte";
    import Header from "./Header.svelte";
    import MainView from "./MainView.svelte";

    import { THEME } from "./Constants";

    import { onMount } from "svelte";

    let d = document.documentElement;
    let themeStorage;

    const handleClick = () => {
        if (d.classList.contains("theme-dark")) {
            d.classList.remove("theme-dark");
            localStorage.removeItem(THEME);
        } else {
            d.classList.add("theme-dark");
            localStorage.setItem(THEME, "dark");
            themeStorage = "dark";
        }
    };

    onMount(async () => {
        themeStorage = localStorage.getItem(THEME);

        if (themeStorage === "dark") {
            d.classList.add("theme-dark");
        }
    });
</script>

<svelte:head>
    <title>Today</title>
</svelte:head>

<main>
    <Header {handleClick} />
    <MainView />
    <Hello />
    <!--    <Login />-->
</main>

<style global>
    @tailwind base;
    @tailwind components;
    @tailwind utilities;

    * {
        font-family: 'rubik';
    }
</style>

