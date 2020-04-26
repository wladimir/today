import Home from './pages/Home.svelte';
import SignIn from './pages/SignIn.svelte';
import Error from './pages/Error.svelte';

export default [
  {
    path: '/',
    component: Home,
    auth: false,
  },
  {
    path: '/signin',
    component: SignIn,
    auth: false,
  },
  {
    path: '*',
    component: Error,
  },
];
