import Home from './pages/Home.svelte';

export default [
  {
    path: '/',
    component: Home,
    auth: false,
  },
  {
    path: '*',
    component: Error,
  },
];
