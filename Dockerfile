FROM alpine:3.14.0 AS assets

ENV TAILWIND_VERSION=3.0.23
ENV TAILWIND_URL=https://github.com/tailwindlabs/tailwindcss/releases/download/v${TAILWIND_VERSION}/tailwindcss-linux-x64

ENV ESBUILD_VERSION=0.14.27
ENV ESBUILD_URL=https://registry.npmjs.org/esbuild-linux-64/-/esbuild-linux-64-${ESBUILD_VERSION}.tgz

WORKDIR /app

# Download Tailwind
RUN \
    apk add curl && \
    curl -sL ${TAILWIND_URL} -o /usr/bin/tailwindcss && \
     chmod +x /usr/bin/tailwindcss

# Download esbuild
RUN \
    apk add curl && \
    curl ${ESBUILD_URL} | tar xvz && \
     mv ./package/bin/esbuild /usr/bin/esbuild && \
     chmod +x /usr/bin/esbuild

# Copy emojied template
# Need this because Tailwind purges the classes in the template. So, without it,
# only the default styles will remain.
COPY src src

# Copy swoogle static files
COPY assets assets
COPY public public

RUN apk add tree

RUN tree .

# Build and minify stylesheet
RUN \
    tailwindcss \
      --input assets/app.css \
      --output public/app.css \
      --config assets/tailwind.config.js \
      --minify

# Build and minify *S
RUN \
    esbuild assets/app.ts \
      --outfile=public/app.js \
      --minify

################################################################################

FROM alpine:3.14.0

WORKDIR /app

RUN chown nobody /app

COPY bin/run run
COPY --chown=nobody:root target/x86_64-unknown-linux-musl/release/emojied emojied
COPY --from=assets --chown=nobody:root /app/public public

EXPOSE 3000

CMD ["/app/run"]
