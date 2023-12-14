echo "Build and run..."
echo "-------------------"
echo " "
rm -rf build/



mkdir ./build
cp src/main.rs ./build/main.rs
cd ./build

rustc main.rs
rm main.rs

./main

rm -rf build/

