<script></script>

<div>
    <img src="/img/chair.jpg" alt="">
    <img src="/img/sauna.jpg" alt="">
    <img src="/img/poker.jpg" alt="">
</div>

<style lang="scss">
    $width: 250px;
    $height: 250px;

    div {
        border: $--c1;
        border-radius: $--border-radius;
        width: $width;
        height: $height;
        box-shadow: 0 0 5px $--c1;
        border: $--border-width $--c2 solid;
        overflow: hidden;
        position: relative;
    }

    img {
        width: $width;
        height: $height;
        position: absolute;
        animation: cycle 45s ease-in-out infinite;
        z-index: 0;
    }

    img:nth-of-type(1) {
        animation-delay: 0s;
    }

    img:nth-of-type(2) {
        animation-delay: -15s;
    }

    img:nth-of-type(3) {
        animation-delay: -30s;
    }

    @keyframes cycle {
        0% {
            z-index: 1;
            top: -250px;
        }

        60% {
            z-index: 1;
            top: -250px;
        }

        65% {
            z-index: 1;
            top: 0px;
        }

        70% {
            z-index: 1;
        }

        70.00001% {
            z-index: 0;
        }
    }
</style>