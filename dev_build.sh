set -x

# ======================================================================================================================
# Compiling all of the packages down to WASM.
# ======================================================================================================================

cd ./1-token-creation/ \
    && scrypto build \
    && cp ./target/wasm32-unknown-unknown/release/token_creation.wasm ./package.wasm \
    && cd ..
cd ./2-token-sale/ \
    && scrypto build \
    && cp ./target/wasm32-unknown-unknown/release/token_sale.wasm ./package.wasm \
    && cd ..
cd ./3-authenticated-token-sale/ \
    && scrypto build \
    && cp ./target/wasm32-unknown-unknown/release/authenticated_token_sale.wasm ./package.wasm \
    && cd ..

# ======================================================================================================================
# Getting the environment variables for the "Token Creation" package
# ======================================================================================================================

cd ./1-token-creation/

# Resetting resim to get a new clean environment
resim reset

# Creating an seller account and a user account to use for the testing of the token sale
OP1=$(resim new-account)
export seller_private_key=$(echo "$OP1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export seller_account=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

# Publishing the package to resim
export package=$(resim publish ./package.wasm | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")

# Instantiating a new component and getting the component and resource addresses
CP_OP=$(resim call-function $package TokenSale new)
export component=$(echo "$CP_OP" | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
export useful_token=$(echo "$CP_OP" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '1!d')

echo "\
export xrd=030000000000000000000000000000000000000000000000000004

export seller_private_key=$seller_private_key
export seller_account=$seller_account

export package=$package

export component=$component
export useful_token=$useful_token" > vars.sh
echo "\
\$xrd=\"030000000000000000000000000000000000000000000000000004\"

\$seller_private_key=\"$seller_private_key\"
\$seller_account=\"$seller_account\"

\$package=\"$package\"

\$component=\"$component\"
\$useful_token=\"$useful_token\"" > vars.ps1

cd ..

# ======================================================================================================================
# Getting the environment variables for the "Authorized Token Sale" package
# ======================================================================================================================

cd ./2-token-sale/

# Resetting resim to get a new clean environment
resim reset

# Creating an seller account and a user account to use for the testing of the token sale
OP1=$(resim new-account)
export seller_private_key=$(echo "$OP1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export seller_account=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

OP2=$(resim new-account)
export buyer_private_key=$(echo "$OP2" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export buyer_account=$(echo "$OP2" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

# Publishing the package to resim
export package=$(resim publish ./package.wasm | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")

# Instantiating a new component and getting the component and resource addresses
CP_OP=$(resim call-function $package TokenSale new 0.5)
export component=$(echo "$CP_OP" | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
export useful_token=$(echo "$CP_OP" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '1!d')

echo "\
export xrd=030000000000000000000000000000000000000000000000000004

export seller_private_key=$seller_private_key
export seller_account=$seller_account

export buyer_private_key=$buyer_private_key
export buyer_account=$buyer_account

export package=$package

export component=$component
export useful_token=$useful_token" > vars.sh
echo "\
\$xrd=\"030000000000000000000000000000000000000000000000000004\"

\$seller_private_key=\"$seller_private_key\"
\$seller_account=\"$seller_account\"

\$buyer_private_key=\"$buyer_private_key\"
\$buyer_account=\"$buyer_account\"

\$package=\"$package\"

\$component=\"$component\"
\$useful_token=\"$useful_token\"" > vars.ps1

cd ..

# ======================================================================================================================
# Getting the environment variables for the "Authenticated Token Sale" package
# ======================================================================================================================

cd ./3-authenticated-token-sale/

# Resetting resim to get a new clean environment
resim reset

# Creating an seller account and a user account to use for the testing of the token sale
OP1=$(resim new-account)
export seller_private_key=$(echo "$OP1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export seller_account=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

OP2=$(resim new-account)
export buyer_private_key=$(echo "$OP2" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export buyer_account=$(echo "$OP2" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

# Publishing the package to resim
export package=$(resim publish ./package.wasm | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")

# Instantiating a new component and getting the component and resource addresses
CP_OP=$(resim call-function $package TokenSale new 0.5)
export component=$(echo "$CP_OP" | sed -nr "s/└─ Component: ([[:alnum:]_]+)/\1/p")
export useful_token=$(echo "$CP_OP" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '1!d')
export seller_badge=$(echo "$CP_OP" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '2!d')

echo "\
export xrd=030000000000000000000000000000000000000000000000000004

export seller_private_key=$seller_private_key
export seller_account=$seller_account

export buyer_private_key=$buyer_private_key
export buyer_account=$buyer_account

export package=$package

export component=$component
export useful_token=$useful_token
export seller_badge=$seller_badge" > vars.sh
echo "\
\$xrd=\"030000000000000000000000000000000000000000000000000004\"

\$seller_private_key=\"$seller_private_key\"
\$seller_account=\"$seller_account\"

\$buyer_private_key=\"$buyer_private_key\"
\$buyer_account=\"$buyer_account\"

\$package=\"$package\"

\$component=\"$component\"
\$useful_token=\"$useful_token\"
\$seller_badge=\"$seller_badge\"" > vars.ps1

echo "\
CALL_METHOD ComponentAddress(\"$seller_account\") \"create_proof\" ResourceAddress(\"$seller_badge\");
CALL_METHOD ComponentAddress(\"$component\") \"change_price\" Decimal(\"10\");" > change_price.rtm

cd ..