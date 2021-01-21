#!/bin/bash


if [ ! -f "data/ignore/wine.csv" ]
then
    curl -o "data/ignore/wine.csv" https://raw.githubusercontent.com/python-engineer/pytorchTutorial/master/data/wine/wine.csv
else
    echo "wine.csv already exists"
fi


if [ ! -d "data/ignore/hymenoptera_data" ]
then
    if [ ! -f "data/ignore/hymenoptera_data.zip" ]
    then
        curl -o "data/ignore/hymenoptera_data.zip" https://download.pytorch.org/tutorial/hymenoptera_data.zip
    else
        echo "hymenoptera_data.zip already present"
    fi
    unzip data/ignore/hymenoptera_data.zip -d data/ignore/
    rm "data/ignore/hymenoptera_data.zip"
else
    echo "hymenoptera_data already present"
fi
