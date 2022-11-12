<script context="module">
 export const ssr = false;
</script>
<script type="ts">
  import { onMount } from "svelte";
  import Post from './Post.svelte';

  let posts = {};
  let ownerId: string;
  let userIds: string[] = [];
  let users = {};
  let includedUsers = {};
  let includedTweets = {};
  let currentUserId: string|null = null;

  onMount(async () => {
    await listUser();
  });

  async function listUser() {
    const acctResp = await fetch("/api/accounts");
    const json = await acctResp.json();
    ownerId = json.owner_id;
    userIds = json.user_ids;

    for (const id of userIds) {
      setUserInfo(id);
    }
  }

  async function addUser() {
    const resp = await fetch(`/api/accounts/add`);
    const json = (await resp.json()).body;
    console.log(json);

    await listUser();
  }

  // api_paramsを含まないと肩が一意に定まらない
  async function setUser(userId: string) {
    currentUserId = userId;

    const resp = await fetch(`/api/timeline?user_id=${userId}`);
    const json = (await resp.json()).body;
    posts[userId] = json.data;
    includedUsers = json.includes.users.reduce((ob, u) => {
      ob[u.id] = u;
      return ob;
    }, includedUsers);
    includedTweets = json.includes.tweets.reduce((ob, u) => {
      ob[u.id] = u;
      return ob;
    }, includedTweets);
    console.log(json);
    posts = posts;
  }

  async function setUserInfo(userId: string) {
    const resp = await fetch(`/api/userinfo?user_id=${userId}`);
    users[userId] = (await resp.json()).body.data;
    users = users;
    userIds = userIds;
  }

  function getUsername(userId: string) {
    return (users[userId] ? '@' + users[userId].username : userId);
  }
</script>

<header>
  <nav>
    <ul>
      <li>minichotan</li>
        {#each userIds as id}
          <li><a href="#" on:click={setUser(id)}>{getUsername(id)}</a></li>
        {/each}
      <li><a href="#" on:click={addUser}>Add Account</a></li>
      <li><a href="/about">about</a></li>
    </ul>
  </nav>
</header>

<main>
  {#if currentUserId && posts[currentUserId]}
    {#each posts[currentUserId] as post}
      <Post {post} users={includedUsers} refTweets={includedTweets} />
    {/each}
  {/if}
</main>

<style>
body {
  margin: 0 !important;
  padding: 0;
  border: none;
}

header {
}

nav {
  margin-bottom: 0.5rem;
  font-size: 0.8rem;
  color: #333;
}

nav ul {
  display: inline;
  margin: 0;
  padding: 0;
}

nav li {
  display: inline;
  margin: 0 0.5rem;
}

nav a {
  text-decoration: none;
}

nav a:hover {
  text-decoration: underline;
}

main {
  padding: 0;
}

</style>
