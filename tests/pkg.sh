cd ../
echo "========wasm-build start========="
wasm-pack build --target nodejs
echo "wasm build done!"
echo "=========wasm-build end=========="
cd ./tests
echo "========update pkg start========="
rm -rf ./pkg/html2VD.js
rm -rf ./pkg/html2VD_bg.wasm
echo "clean pkg done!"
cp ../pkg/html2VD.js ./pkg/
cp ../pkg/html2VD_bg.wasm ./pkg/
echo "update pkg done!"
echo "=========update pkg end=========="
