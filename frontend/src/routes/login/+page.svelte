<script lang="ts">
  import { onMount } from 'svelte';
  import { dev } from '$app/environment';
  import 'crypto';

  // TODO: do not hardcode but who am i kidding this is going to be in production i guarantee it
  const CLIENT_ID = 'ee92s9l7bxh4fslbqh3svb3ul7hmfi';
  const REDIRECT_URI = 'http://localhost:5173/login';
  const TOKEN_POST_URI = 'http://127.0.0.1:3030/token';

  function buildURL(client_id: string, redirect_uri: string, state: string) {
    return `https://id.twitch.tv/oauth2/authorize?response_type=token&client_id=${client_id}&redirect_uri=${redirect_uri}&scope=&state=${state}`;
  }

  onMount(async () => {
    const urlParams = new URLSearchParams(window.location.search);
    const error = urlParams.get('error');
    if (error) {
      alert('Could not authenticate');
      return;
    }

    const hashParams = new Map(
      window.location.hash
        .replace('#', '')
        .split('&')
        .map((val) => val.split('=')) as [string, string][]
    );
    console.log(hashParams);

    const accessToken = hashParams.get('access_token');
    if (!accessToken) {
      console.log('access token not found');
      const state = crypto.randomUUID();
      window.location.href = buildURL(CLIENT_ID, REDIRECT_URI, state);
      // window.history.pushState({}, '', buildURL(CLIENT_ID, REDIRECT_URI, state));
      if (dev) {
        // disable secure if dev
        document.cookie = `auth_state=${state}; SameSite=strict`;
      } else {
        document.cookie = `auth_state=${state}; SameSite=strict; Secure`;
      }

      return;
    }

    const cookieState = document.cookie
      .split('; ')
      .find((row) => row.startsWith('auth_state='))
      ?.split('=')[1];

    if (cookieState !== hashParams.get('state')) {
      alert('Try again');
      console.warn('Cookie state does not match state returned by twitch');
      return;
    }

    console.log('ideally here we would post token to the backend');

    // TODO: send access token to backend
    try {
      const response = await fetch(TOKEN_POST_URI, {
        method: 'POST',
        body: accessToken
      });

      const response_as_json = await response.json();
      if (dev) {
        document.cookie = `session_token=${response_as_json}; SameSite=strict`;
      } else {
        document.cookie = `session_token=${response_as_json}; SameSite=strict; Secure`;
      }

      window.history.pushState({}, '', '/');
      window.location.href = '/';
    } catch (e) {
      console.warn('Error when posting token', e);
    }
  });
</script>
