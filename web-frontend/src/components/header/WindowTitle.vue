<template>
	<div class="window-title">
		<span>X-35B.gdd* - Graphite</span>
	</div>
</template>

<style lang="scss">
.window-title {
	display: flex;
	align-items: center;
	padding: 0 8px;
}
</style>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({
	components: {},
	props: {},
})
export default class WindowTitle extends Vue {}
</script>
