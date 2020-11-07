#!/bin/bash

main() {
		# First generate the genesis state, with the SRM/MSRM mints and
		# funded wallet (at ~/.config/solana/id.json).
		#
		# Example `genesis` var:
		#
    # Genesis {
    #     wallet: FhmUh2PEpTzUwBWPt4qgDBeqfmb2ES3T64CkT1ZiktSS,
    #     mint_authority: FhmUh2PEpTzUwBWPt4qgDBeqfmb2ES3T64CkT1ZiktSS,
    #     god_owner: FhmUh2PEpTzUwBWPt4qgDBeqfmb2ES3T64CkT1ZiktSS,
		#     srm_mint: E7ScVS17ak1ZVy9nNyGsVqZ48QdcDgdxSk1wXfD8zW3o,
		#     msrm_mint: 4ozqYu5Qjz8W9hfqXDA4XZNdEARCHuWGU4v8eSYX1XDQ,
		#     god: HWT4vz4u2KdkimMDoMS96HSeJLGTpfzScU44PKWpG7D,
		#     god_msrm: 7J2HeEnbfugJN8uiyPrczk8rWMQG6gVBF1zE1g4gdyqZ,
		#     god_balance_before: 1000000000000000,
		#     god_msrm_balance_before: 1000000000000000,
		# }
		#
		local genesis=$(cargo run -p serum-node -- -c l dev init-mint)
    local srm_mint=$(echo $genesis | sed 's/.*{.* srm_mint: \(.*\),.*msrm_mint.*}.*/\1/g')
    local msrm_mint=$(echo $genesis | sed 's/.*{.* msrm_mint: \(.*\),.*god:.*}.*/\1/g')
		local god=$(echo $genesis | sed 's/.*{.* god: \(.*\),.*god_msrm:.*}.*/\1/g')
		local god_msrm=$(echo $genesis | sed 's/.*{.* god_msrm: \(.*\),.*god_balance_before:.*}.*/\1/g')

		pids=$(make -s -C registry deploy-all)
		registry_pid=$(echo $pids | jq .registryProgramId -r)
		stake_pid=$(echo $pids | jq .stakeProgramId -r)
		lockup_pid=$(echo $pids | jq .lockupProgramId -r)

		echo $pids
		echo $registry_pid
		echo $stake_pid
		echo "god: SRM/MSRM"
		echo $god
		echo $god_msrm

		cargo run -p serum-node -- \
					-c l \
					--srm-mint $srm_mint \
					--msrm-mint $msrm_mint \
					registry --pid $registry_pid \
					init \
					--pool-program-id $stake_pid \
					--pool-token-decimals 3 \
					--deactivation-timelock 5 \
					--reward-activation-threshold 1 \
					--withdrawal-timelock 2
}

main
