# Intro To Scrypto

## Structure

This repository contains the source code for the Token Sale Blueprint as a way to get a flavor of how it feels to develop with Scrypto. The three packages included here are designed to walk you through multiple stages of developing a blueprint, up until a point where this blueprint has all of the important pieces in place. 

The following are the topics that each of the three packages are based on:

|   | Example Name   | Topics |
| - | -------------- | ------ |
| 1 | Token Creation | This package is designed to walk you through the creation of a simple token in Scrypto. It introduces the idea of the [`ResourceBuilder`](https://radixdlt.github.io/radixdlt-scrypto/scrypto/resource/struct.ResourceBuilder.html) as well as some of the options available on the resource builder. This is the first time that you see vaults but there is no vault interactions that occur here. | 
| 2 | Token Sale     | With the token created, we would now like to accept some kind of XRD payments in exchange for our token: this package walks you through precisely just that. This package offers a more detailed example of how different vaults can be used in a component and how you can perform simple token exchanges through your component. | 
| 3 | Authenticated Token Sale | At this point, we have created a token, have created a simple way to sell the token, but there is some functionality missing from our dApp, namely: the withdrawal of funds from the component and changing the price of tokens. This package introduces you to authorization in Scrypto and how badges can be used to allow you to perform admin-only actions. |

## Goals

By the end of this webinar, you should have practical Scrypto knowledge of the following concepts:

1. Blueprints and components.
1. Resources and how to create them.
1. Vaults and how they can be manipulated and have funds put into them and taken out of them.
1. Writing blueprint methods and functions.
1. Badges and their relationship to resources and the auth system.

## Written Explanation

We would like to build a complete token sale from beginning to end, this includes the process of creating a new token, creating a new blueprint to handle the token sale, allowing people to exchange their XRD for this new token, and allowing the creator of the token to withdraw the funds that they got from the token sale. 

Lets first begin with a skeleton blueprint called `TokenSale` that has just enough to be called a blueprint. 

```rust
use scrypto::prelude::*;

#[blueprint]
mod token_sale {
    struct TokenSale {

    }

    impl TokenSale {
        pub fn instantiate_token_sale() {

        }
    }
}
```

Let's begin thinking about what exactly do we need the blueprint to have and what state would we like it to have. For a fixed-supply token, what things does the sale component need to know about the token or have for the token? It only needs to have a vault which stores the tokens so that they are immediately available when we would like to sell them. So, we will be adding a `useful_tokens_vault` vault to the blueprint state

```rust
struct TokenSale {
    useful_tokens_vault: Vault
}
```

Now that we have somewhere for the tokens to live when they are created, we can actually go ahead and create the new token. We would like the new token that we create to be called "UsefulToken" and have the symbol `UT`. So, we will change the `new` function body to be the following:

```rust
pub fn instantiate_token_sale() -> ComponentAddress {
    // Creating a new token called "UsefulToken"
    let my_bucket: Bucket = ResourceBuilder::new_fungible()
        .metadata("name", "UsefulToken")
        .metadata("symbol", "USEFUL")
        .mint_initial_supply(1000);

    // Creating a new component and storing the tokens in the component's vault.
    Self {
        useful_tokens_vault: Vault::with_bucket(my_bucket)
    }
    .instantiate()
    .globalize()
}
```

At this point, we have a function on the `TokenSale` blueprint which is called `new`. This function has two main responsibilities:

1. The creation of the new token that we are calling "Useful Token".
2. Instantiating a new `TokenSale` component with a vault containing all of the "Useful Token"s that we created.

This is all that we need to do to create a simple token in Scrypto! We would now like to add some additional and exciting concepts in there as well. What we would like to do now is to allow for users to buy our tokens from the component. Therefore, we need to have a state variable defining the current price of the tokens and a vault to store our XRD in. Therefore, the new `TokenSale` struct would look like the following:

```rust
struct TokenSale {
    // A vault where the "Useful Token"s will be stored.
    useful_tokens_vault: Vault,

    // A vault where the XRD from the payments will be stored.
    xrd_tokens_vault: Vault,

    // The current price of a single UsefulToken in terms of XRD
    price_per_token: Decimal
}
```

Since we have changed our `TokenSale` struct, we also need to make changes to our `instantiate_token_sale` function so that the body correctly instantiates a valid struct. There will be two main changes which we need to make:

1. We need to add an argument for the price so that the instantiator of the component would have the ability to set the price of tokens at instantiation-time. 
2. We have added two new state variables: `xrd_tokens_vault`, and `price_per_token`. These two state variables need to be initialized at component instantiation. 

Therefore, the `instantiate_token_sale` function on the `TokenSale` blueprint would now look like the following:

```rust
pub fn instantiate_token_sale(price_per_token: Decimal) -> ComponentAddress {
    // Creating a new token called "UsefulToken"
    let my_bucket: Bucket = ResourceBuilder::new_fungible()
        .metadata("name", "UsefulToken")
        .metadata("symbol", "USEFUL")
        .mint_initial_supply(1000);

    // Creating a new component and storing the tokens in the component's vault.
    Self {
        useful_tokens_vault: Vault::with_bucket(my_bucket),
        xrd_tokens_vault: Vault::new(RADIX_TOKEN),
        price_per_token: price_per_token
    }
    .instantiate()
    .globalize()
}
```

| **Note** | Notice how the XRD vault is created through `Vault::new` while the useful tokens vault is created through `Vault::with_bucket`. These two functions allow us to either create an empty vault, or to create a vault with some initial amount that is obtained from a bucket. |
| -------- | :--- |

We now have a way of storing the XRD sent to the component, and we have a function used to create a new `TokenSale` component. The only thing that is remaining for the token sale is the method which performs the actual sale of tokens. Let's call this method `buy`. Users would call this method with a bucket of XRD, based on that it would determine how much tokens can be bought and returns them to the user. This method would look like the following:

```rust
pub fn buy(&mut self, funds: Bucket) -> Bucket {
    let purchase_amount: Decimal = funds.amount() / self.price_per_token;
    self.xrd_tokens_vault.put(funds);
    self.useful_tokens_vault.take(purchase_amount)
}
```

With that, we have built a very simple token sale blueprint which creates a new token on instantiation and allows for users to call the `buy` method to purchase the "Useful Token"s. However, there is something that we have not yet added into the blueprint: a way for the seller to withdraw their tokens from the component. To build this, we would need to use badges to authenticate the seller so that only they can withdraw the XRD obtained from the sale and no body else can do that. In addition to that, it would be useful to have an authenticated method which allows the seller to change the price of the tokens at some point in the future. So in summary, we would like to add two authenticated methods which perform the following:

1. `withdraw_funds`: An authenticated method which can only be called by the seller to withdraw the funds that they have obtained so far from the sale.
2. `change_price`: An authenticated method which can only be called by the seller to change the price of the tokens in the token sale. 

With the function names and functionality clearly defined, we can dive into the implementation of these two methods. These methods are quite straightforward and their bodies are just a single line of code, they are defined as follows:

```rust
pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
    self.xrd_tokens_vault.take(amount)
}

pub fn change_price(&mut self, price: Decimal) {
    self.price_per_token = price;
}
```

We have defined our methods but we now have a problem: anybody can call these methods and not just the seller. To do that we would do the following:

1. Create a new resource which we would be providing to the seller only. This resource would act as the seller's badge and would be used to authenticate them. The creation of this seller badge resource happens in the `instantiate_token_sale` function just like the "Useful Token". The code for it is as follows:
    
    ```rust
    pub fn instantiate_token_sale(price_per_token: Decimal) -> ComponentAddress {
            // Creating a new token called "UsefulToken"
            let my_bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "UsefulToken")
                .metadata("symbol", "USEFUL")
                .mint_initial_supply(1000);

            // ---- This is the new badge we're creating in this step ----
            let seller_badge: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Seller Badge")
                .metadata("symbol", "SELLER")
                .mint_initial_supply(1);

            Self {
                useful_tokens_vault: Vault::with_bucket(my_bucket),
                xrd_tokens_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token
            }
            .instantiate()
            .globalize()
        }
    ```

2. With our new badge created, we need to inform the Radix Engine that it should require that the seller badge is present when the `withdraw_funds` or `change_price` methods are called. This is done by defining the access rules through using Scrypto's native concept called the `AccessRules` struct. When we've defined our access rules, to implement it, we need to now change the way we refer to our component by creating a mutable variable. This is because implementing access rules to our component requires changing our component. After instantiation of the component we then assign our component with the `.add_access_check()` method. The code for this looks like the following:

    ```rust
    pub fn instantiate_token_sale(price_per_token: Decimal) -> (ComponentAddress, Bucket) {

        // Creating a new token called "UsefulToken"
        let my_bucket: Bucket = ResourceBuilder::new_fungible()
            .metadata("name", "UsefulToken")
            .metadata("symbol", "USEFUL")
            .mint_initial_supply(1000);

        // Creating a new seller badge which we will give the withdraw authority to
        let seller_badge: Bucket = ResourceBuilder::new_fungible()
            .metadata("name", "Seller Badge")
            .metadata("symbol", "SELLER")
            .mint_initial_supply(1);

        // Setting the access rules to only allow the seller badge to withdraw the funds or change the price
        let access_rules: AccessRules = AccessRules::new()
            .method("withdraw_funds", rule!(require(seller_badge.resource_address())), LOCKED)
            .method("change_price", rule!(require(seller_badge.resource_address())), LOCKED)
            .default(rule!(allow_all), LOCKED);

        let mut token_sale: TokenSaleComponent = Self {
            useful_tokens_vault: Vault::with_bucket(my_bucket),
            xrd_tokens_vault: Vault::new(RADIX_TOKEN),
            price_per_token: price_per_token
        }
        .instantiate();
        token_sale.add_access_check(access_rules);
        let token_sale_component_address: ComponentAddress = token_sale.globalize();

        return (token_sale_component_address, seller_badge)
    }
    ```

Note that we also created a variable to refer to the component's `ComponentAddress`. And with that we are done! We have created our full token sale blueprint which allows for tokens to be sold to users and also allows for privileged operations such as the withdrawal of XRD from the token sale component and the changing of the token prices. 

## Running the Examples

| **On Windows** | You should run the examples using the PowerShell and run the following command first: `Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope Process`  |
| -------- | :--- |

### Token Creation

This section takes you through how a token can be created in Scrypto through a blueprint. 


1. Change your current working directory to be that of the example. Since this walk through is for the token creation example, then run the following command to change into that directory:

    ```sh
    cd ./1-token-creation
    ```

1. We are now ready to go through the example. The first thing that we would like to do for this example is to create a new account (which we are calling the admin account) which will be instantiating the `TokenSale` component. We can do that through:

    ```sh
    resim new-account
    ```

The terminal should return something that looks like this (your values may look a little different):

    ```sh
    A new account has been created!
    Account component address: account_sim1qwv7t2pd073njrev2nazff23et4jrfsq0vp39thrr5hqfa037x
    Public key: 0246d5370b7cb66623bac934051cb92aa9e83b3fd1ccb9ae338e85b898249b53e6
    Private key: b54b125f5519f202e5edf5b1b3d8a845b894bd1e7f0ee32f734b1fd676f8a146
    NonFungibleGlobalId: resource_sim1qqadydy0qj2mrfghfshvaxsk2yjj5ur3l02rj3gsjjxqw8n2vl:#1#
    ```

    For later convenience, let's save these values to our environment variable with the following command:

    **Windows**
    ```sh
    $account1="account_sim1qwv7t2pd073njrev2nazff23et4jrfsq0vp39thrr5hqfa037x"
    $pub_key1="0246d5370b7cb66623bac934051cb92aa9e83b3fd1ccb9ae338e85b898249b53e6"
    $priv_key1="b54b125f5519f202e5edf5b1b3d8a845b894bd1e7f0ee32f734b1fd676f8a146"
    $owner_badge1="resource_sim1qqadydy0qj2mrfghfshvaxsk2yjj5ur3l02rj3gsjjxqw8n2vl:#1#"
    ```

    **MacOS or Linux**
    ```sh
    export account1=account_sim1qwv7t2pd073njrev2nazff23et4jrfsq0vp39thrr5hqfa037x
    export pub_key1=0246d5370b7cb66623bac934051cb92aa9e83b3fd1ccb9ae338e85b898249b53e6
    export priv_key1=b54b125f5519f202e5edf5b1b3d8a845b894bd1e7f0ee32f734b1fd676f8a146
    export owner_badge1=resource_sim1qqadydy0qj2mrfghfshvaxsk2yjj5ur3l02rj3gsjjxqw8n2vl:#1#
    ```

1. We are now ready to push our package to our local simulator. This is done through:

    ```sh
    resim publish .
    ```

1. With the package published to the local simulator, you should receive a package address from your terminal. Let's also save the package address to our local environment as `package`.

1. Since the package has been locally deployed, the blueprints included there can be called and components of these blueprints can be instantiated. We will be instantiating a new `TokenSale` component and with it will come the token that we are creating. The component can be instantiated through: 

    ```sh
    resim call-function $package TokenSale instantiate_token_sale
    ```

1. We have now created a component and a new token!

### Token Sale

This section takes you through how a token can be created in Scrypto through a blueprint and sold.


1. Change your current working directory to be that of the example. Since this walk through is for the token sale example, then run the following command to change into that directory:

    ```sh
    cd ./2-token-sale
    ```

1. Let's first reset our local ledger to erase anything that was saved from our previous work:

    ```sh
    resim reset
    ```

1. We are now ready to go through the example. The first thing that we would like to do for this example is to create a two new accounts which we will be using for the seller and the buyer. We can do that through:

    ```sh
    resim new-account
    resim new-account
    ```

Just like before, we will create environment variables for convenience. However, I personally would like to change the environment variable naming to add more clarity:
        
        **Windows**
    ```sh
    $seller_account=""
    $seller_pubkey=""    
    $seller_privkey=""
    $owner_badge=""
    ```
    ```sh
    $buyer_account
    $buyer_pubkey    
    $buyer_privkey
    ```

    **MacOS or Linux**
    ```sh
    export seller_account=
    export seller_pubkey=
    export seller_privkey=
    export owner_badge=
    ```
    ```sh
    export buyer_account=
    export buyer_pubkey=
    export buyer_privkey=
    ```

1. We are now ready to push our package to our local simulator. This is done through:

    ```sh
    resim publish .
    ```

1. With the package published to the local simulator, the blueprints included there can be called and components of these blueprints can be instantiated. We will be instantiating the `TokenSale` component and with it will come the token that we are creating. The component can be instantiated through: 

    ```sh
    resim call-function $package TokenSale instantiate_token_sale 0.5
    ```

1. We have now created a new component and token! Let's also save the `ComponentAddress` that was returned to an environment variable `component`. We can now attempt to buy some of those tokens from the buyer's account. Lets first switch to the buyer's account through: 

    ```sh
    resim set-default-account $buyer_account $buyer_privkey $owner_badge
    ```

1. With the account switched, we can now buy some tokens! We will do that through: 

    ```sh
    resim call-method $component buy 300,resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqz8qety
    ```

### Authenticated Token Sale

This section takes you through how a token can be created in Scrypto through a blueprint and sold. In addition to that, there are a number of authenticated methods which are implemented on this component which allow the seller to change some parameters later on.


1. Change your current working directory to be that of the example. Since this walk through is for the authenticated token sale example, then run the following command to change into that directory:

    ```sh
    cd ./3-authenticated-token-sale
    ```

1. The first thing that you need to do is to setup your environment variables with the script files that we have provided. Lets first reset resim through:

    ```sh
    resim reset
    ```

1. We are now ready to go through the example. The first thing that we would like to do for this example is to create a two new accounts which we will be using for the seller and the buyer. We can do that through:

    ```sh
    resim new-account
    resim new-account
    ```

    As always, it is convenient to save the values to the environment variables, but it is not necessary!

1. We are now ready to push our package to our local simulator. This is done through:

    ```sh
    resim publish .
    ```

1. With the package published to the local simulator, the blueprints included there can be called and components of these blueprints can be instantiated. We will be instantiating a new `TokenSale` component and with it will come the token that we are creating. The component can be instantiated through: 

    ```sh
    resim call-function $package TokenSale new 0.5
    ```
    The difference in the output here is that we received a new resource address. Save the last resource address value to `admin_badge` as an environment variable.

1. Lets now assume that the seller wishes to change the price of their tokens from 0.5 XRD per token to 10 XRD per token, we can do that with this command:

    ```sh
    resim call-method $component change_price 10 --proofs 1,$admin_badge
    ```

1. With the price changed to 10 XRD per token instead of 0.5 per token, we can now attempt to purchase the tokens as the buyer and then examine how much tokens we get. We can do that through:

    ```sh
    resim set-default-account $buyer_account $buyer_privkey $owner_badge
    resim call-method $component buy 300,$xrd
    resim show $buyer_account
    ```

    We can see that our account has got 30 useful tokens in our account as a result of the purchase. 
