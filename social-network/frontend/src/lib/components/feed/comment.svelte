<script>
    export let comment
    let postDate = new Date(comment.created).toLocaleString()

    function openImage(e) {
        window.open(e.target.src, '_blank');
    }
</script>


<div class="post-div">
	<div>
		<address><a rel="author" href="/user/{comment.author.userID}">{comment.author.firstName}</a> commented at {postDate}:
		</address>
		<article>
			<p>
				{comment.content}
			</p>
			{#if comment.images !== ""}
				<hr/>
				<img src="/api/file/{comment.images}" alt="{comment.images}" on:click={openImage}/>
			{/if}
		</article>

	</div>
</div>


<style lang="scss">
  .read-more {
    margin: 5px 10px;
  }

  .post-div {
    background-color: $--c0-l10;
    border-radius: $--border-radius;
    padding: 0.5em 1em;
    margin: 5px 10px;
  }

  address {
    font-size: 1.2rem;
    opacity: 0.8;
  }

  a {
    text-decoration: none;
    color: $--c2-l5;
    font-weight: bold;
  }

  a:hover {
    text-decoration: underline;
  }

  article {
    margin-left: 0.5em;
    // border-left: 2px solid black;
    padding: 0.1em 0.2em;
    // background-color: rgba(0, 0, 0, 0.05);
  }

  img {
    max-height: 200px;
    cursor: pointer;
  }

  p {
    width: 100%;
    border-radius: 5px;
    //background-color: transparentize($--c0, 0.5);
    //padding: 0.2em 1em;
    font-size: 1.1rem;
  }

  hr {
    margin: 0.5em 0;
  }
</style>
