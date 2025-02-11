if [ -z "$1" ]
	then
		NETWORK=""
	else
		NETWORK="--network $1"
fi
IDENTITY="--identity kong"
PRINCIPAL_ID=$(dfx identity ${IDENTITY} get-principal)

dfx deploy ${NETWORK} ${IDENTITY} event_store --argument "(
    record {
        push_events_whitelist = vec { principal \"${PRINCIPAL_ID}\" };
        read_events_whitelist = vec { principal \"${PRINCIPAL_ID}\"};
        time_granularity = opt (2000:nat64);
    }
)"
