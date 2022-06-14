# TBD WN

## Structure

This repository contains the source code of the TBD WN webinar held on June, 28th 2022. The three packages included here are designed to walk you through multiple stages of developing a blueprint, up until a point where this blueprint has all of the important pieces in place. 

The following are the topics that each of the three packages are based on:

|   | Example Name   | Topics |
| - | -------------- | ------ |
| 1 | Token Creation | This package is designed to walk you through the creation of a simple token in Scrypto. It introduces the idea of the `ResourceBuilder` as well as some of the options available on the resource builder. This is the first time that you see vaults but there is no vault interactions that occur here. | 
| 2 | Token Sale     | With the token created, we would now like to accept some kind of XRD payments in exchange for our token: this package walks you through precisely just that. This package offers a more detailed example of how different vaults can be used in a component and how you can perform simple token exchanges through your component. | 
| 3 | Authenticated Token Sale | At this point, we have created a token, have creates a simple way to sell the token, but there is some functionality missing from our dApp, namely: the withdrawal of funds from the component and changing the price of tokens. This package introduces you to authorization in Scrypto and how badges can be used to allow you to perform only-admin-actions. |

## Goals

By the end of this webinar, you should have practical Scrypto knowledge of the following concepts:

1. Blueprints and components.
1. Resources and how to create them.
1. Vaults and how they can be manipulated and have funds put into them and taken out of them.
1. Writing blueprint methods and functions.
1. Badges and their relationship to resources and the auth system.

## Written Explanation

TODO

## Running the Examples

### Token Creation

This section takes you through how a token can be created in Scrypto through a blueprint. 


1. Change your current working directory to be that of the example. Since this walk through is for the token creation example, then run the following command to change into that directory:

    ```sh
    cd ./1-token-creation
    ```

1. The first thing that you need to do is to setup your environment variables with the script files that we have provided. Lets first reset resim through:

    ```sh
    resim reset
    ```

    Then, depending on what operating system you are on, you would want to run the `vars.sh` or the `vars.ps1` file. If you are on MacOS or Linux, then run the `vars.sh` file through:
    
    ```sh
    source ./vars.sh
    ```

    If you are on Windows (Powershell) then you can set the environment variables through: 

    ```sh
    ./vars.ps1
    ```

1. We are now ready to go through the example. The first thing that we would like to do for this example is to create a new account (which we are calling the admin account) which will be instantiating the `TokenSale` component. We can do that through:

    ```sh
    resim new-account
    ```

1. We are now ready to push our package to our local simulator. This is done through:

    ```sh
    resim publish ./package.wasm
    ```

1. With the package published to the local simulator, the blueprints include there can be called and components of these blueprints can be instantiated. We will be 