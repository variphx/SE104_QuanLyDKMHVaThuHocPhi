<script lang="ts">
  import "../app.postcss";

  // Highlight JS
  import hljs from "highlight.js/lib/core";
  import "highlight.js/styles/github-dark.css";
  import {
    AppBar,
    Avatar,
    Toast,
    initializeStores,
    storeHighlightJs,
  } from "@skeletonlabs/skeleton";
  import xml from "highlight.js/lib/languages/xml"; // for HTML
  import css from "highlight.js/lib/languages/css";
  import javascript from "highlight.js/lib/languages/javascript";
  import typescript from "highlight.js/lib/languages/typescript";

  hljs.registerLanguage("xml", xml); // for HTML
  hljs.registerLanguage("css", css);
  hljs.registerLanguage("javascript", javascript);
  hljs.registerLanguage("typescript", typescript);
  storeHighlightJs.set(hljs);

  // Floating UI for Popups
  import {
    computePosition,
    autoUpdate,
    flip,
    shift,
    offset,
    arrow,
  } from "@floating-ui/dom";
  import { storePopup } from "@skeletonlabs/skeleton";
  storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

  initializeStores();
</script>

<Toast></Toast>

<AppBar background="variant-glass-surface">
  <svelte:fragment slot="lead">(icon)</svelte:fragment>
  <span class="h1 font-bold mx-4"> UIT Management System </span>
  <svelte:fragment slot="trail">
    <Avatar>(ava)</Avatar>
  </svelte:fragment>
</AppBar>

<slot />
