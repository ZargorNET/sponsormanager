module.exports = {
    content: ['./src/**/*.{html,js, vue}'],
    plugins: [
        function ({addVariant}) {
            addVariant('child', '& > *');
            addVariant('child-hover', '& > *:hover');
        }
    ],
    theme: {
        extend: {
            fontFamily: {
                gloock: ["Gloock"]
            },
            boxShadow: {
                innerRing: "inset 0px 1px 2px #a2a2da"
            }
        }
    }
}
