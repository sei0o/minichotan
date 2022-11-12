<script type='ts'>
  // tweet itself
  export let post;
  // users[user_id] = user information;
  export let users: any[string] = {};
  // refTweets[tweet_id] = tweet content;
  export let refTweets: any[string] = {};
  // medium[media_key] = media content
  //  example: {
  //         "height": 946,
  //         "media_key": "3_159113947387430271",
  //         "type": "photo",
  //         "url": "https://pbs.twimg.com/media/XXXXXXXXXX.jpg",
  //         "width": 2048
  //     },
  //     ...
  export let medium = {};

  let json: string = "";

  let opened = false;
  let text: string;
  let originalText: string;
  let escapedText = "";
  $: {
    const t = originalText
      .replace(/https:\/\/t.co\/\w+(\s|$)/g, "")
      .replace(/^RT @([^:])+: /g, "")
      .replace(/\n+$/g, "");
    const doc = new DOMParser().parseFromString(t, "text/html");
    escapedText = doc.documentElement.textContent || "";
  }
  let author = {}; 
  let links: string[] = [];
  let mediaKeys: string[] = [];

  let retweet: boolean, retweetSrc, retweetSrcAuthor;
  let quote: boolean, quoteSrc, quoteSrcAuthor;

  $: originalText = retweet ? retweetSrc.text : text;

  let originalDate, formattedDate;
  $: {
    const d = new Date(originalDate);
    formattedDate = d.toLocaleString();
  }

  $: {
    post = post;

    text = post.text;
    json = JSON.stringify(post, null, "  ");
    author = users[post.author_id];

    if (post.entities && post.entities.urls) {
      links = post.entities.urls.filter(link => !link.media_key || medium[link.media_key].type != "photo");
    } else {
      links = [];
    }

    if (post.attachments) {
      mediaKeys = post.attachments.media_keys;
    } else {
      mediaKeys = [];
    }

    if (post.referenced_tweets) {
      for (const ref of post.referenced_tweets) {
        if (ref.type === "retweeted") {
          retweet = true;
          retweetSrc = refTweets[ref.id];
          retweetSrcAuthor = users[retweetSrc.author_id];
          break;
        } else {
          retweet = false;
        }

        if (ref.type === "quoted") {
          quote = true;
          quoteSrc = refTweets[ref.id];
          quoteSrcAuthor = users[quoteSrc.author_id];
        } else {
          quote = false;
        }
      }
    } else {
      retweet = false;
      quote = false;
    }

    originalDate = retweet ? retweetSrc.created_at : post.created_at;
  }

  function toggle() {
    opened = !opened;
  }
</script>

<div class="post" on:click={toggle} class:open={opened}>
  <span class="text">
    {#if quote}
      {quoteSrc.text}
    {/if}
    {escapedText}<br>
  </span>
  <div class="links">
    {#each links as link}
      <a class="link" href={link.url}>
        {#if link.media_key}
          [Media]
        {:else}
          {link.title || link.display_url}
        {/if}
      </a>
    {/each}
  </div>
  {#if mediaKeys.length > 0}
    <div class="media">
      {#each mediaKeys as key}
        {@const media = medium[key]}
        {#if media.type == "photo"}
          <a href={media.url}>
            <img class="photo" src={media.url} alt={media.alt_text}>
          </a>
        {/if}
      {/each}
    </div>
  {/if}
  <div class="detail">
    {#if author}
      {#if retweet}
        <span class="author">@{retweetSrcAuthor.username} (@{author.username} retweeted)</span>
      {:else}
        <span class="author">@{author.username}</span>
      {/if}
    {/if}
    <span class="date">{formattedDate}</span>
    <span class="debug">{json}</span>
  </div>
</div>

<style>
  .post {
    white-space: pre-line;
    width: 100%;
    border-top: 1px solid #ddd;
    padding: 0.2rem 0;
    min-height: 1rem;
  }

  .debug {
    display: block;
    font-size: 0.6rem;
    font-family: monospace;
    color: #666;
    white-space: pre-wrap;
  }

  .detail {
    display: none;
  }

  .post.open .detail {
    display: block;
    position: relative;
  }

  .author {
    font-size: 0.8rem;
  }

  .date {
    font-size: 0.8rem;
    position: absolute;
    right: 0;
    left: auto;
  }

  .link {
    color: #444;
    margin-right: 0.3rem;
    font-size: 0.8rem;
  }

  .media {
    margin-top: 0.5rem;
  }

  .photo {
    width: 100%;
    max-width: 50rem;
    height: 2rem;
    object-fit: cover;
    border-radius: 4px;
  }

  .post.open .photo {
    height: auto;
    width: auto;
    max-height: 70vh;
    max-width: 100%;
    object-fit: inherit;
  }
</style>
