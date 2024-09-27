#!/bin/bash
current_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
project_dir=$( cd "${current_dir}"/.. && pwd )

target=${1-en}
echo "translating markdown files to $target, react page ignored."

cd "${project_dir}"

# json
npm run write-translations -- --locale en

# markdown
mkdir -p i18n/en/docusaurus-plugin-content-docs/current
cp -rn docs/** i18n/en/docusaurus-plugin-content-docs/current
mkdir -p i18n/en/docusaurus-plugin-content-blog
cp -rn blog/** i18n/en/docusaurus-plugin-content-blog
mkdir -p i18n/en/docusaurus-plugin-content-pages
cp -rn src/pages/**.md i18n/en/docusaurus-plugin-content-pages

# npm run start -- --locale en
