module.exports = {
    theme: {
        extend: {
            colors: {},
            textColor: {
                primary: 'var(--color-text-primary)',
                secondary: 'var(--color-text-secondary)',
                tertiary: 'var(--color-text-tertiary)',
            },
            backgroundColor: {
                primary: 'var(--color-bg-primary)',
                secondary: 'var(--color-bg-secondary)',
                tertiary: 'var(--color-bg-tertiary)',
            },
            fontFamily: {
                display: 'var(--font-display)',
                body: 'var(--font-body)'
            },
            fontWeights: {
                normal: 'var(--font-weight-normal)',
                display: 'var(--font-weight-display)',
                btn: 'var(--font-weight-btn)'
            },
            borderRadius: {
                none: '0',
                btn: 'var(--rounded-btn)'
            }
        }
    },
    variants: {},
    plugins: []
};
