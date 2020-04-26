<script>
  import Hello from './components/Hello.svelte';
  import Header from './components/Header.svelte';
  import Home from './pages/Home.svelte';

  import { THEME } from './Constants';

  import { onMount } from 'svelte';

  import router from 'page';
  import routes from './routes.js';

  let page;
  let params;
  let user = true;

  let d = document.documentElement;
  let themeStorage;

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
  });

  routes.forEach((route) => {
    router(
      route.path,

      (ctx, next) => {
        params = ctx.params;
        next();
      },

      () => {
        if (route.auth && !user) {
          router.redirect('/signin');
        } else {
          page = route.component;
        }
      }
    );
  });

  // Start the router
  router.start();
</script>

<!--prettier-ignore-->
<!--suppress CssInvalidAtRule -->
<svelte:head>
  <title>Today</title>
</svelte:head>

<main>
  <svelte:component this="{page}" />

  <Header {handleClick} />
  <Home />
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
