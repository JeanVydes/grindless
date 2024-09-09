/** @type {import('next').NextConfig} */
const nextConfig = {
    experimental: {
        esmExternals: true,
    },
    webpack: (config, { isServer }) => {
        if (!isServer) {
            config.resolve.fallback = {
                ...config.resolve.fallback,
                fs: false,
                stream: false,
                crypto: false,
            };
        }

        config.module.rules.push({
            test: /pdf\.worker\.(min\.)?js/,
            use: [
                {
                    loader: 'file-loader',
                    options: {
                        name: '[name].[ext]',
                        publicPath: '/_next/static/worker',
                        outputPath: 'static/worker',
                    },
                },
            ],
        });

        return config;
    },
};

export default nextConfig;
