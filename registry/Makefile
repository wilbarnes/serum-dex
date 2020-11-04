include ../Makefile

LIB_NAME=serum_registry_program

.PHONY: test test-program deploy-lockup deploy-stake deploy-all

test: deploy-lockup deploy-stake deploy-super test-program
	@ # no-op

deploy-stake:
	$(eval TMP=$(shell make -s -C ../stake deploy))
	$(eval TEST_STAKE_PROGRAM_ID=$(shell echo $(TMP) | sed 's/.*{programId: \(.*\)}.*/\1/g'))

deploy-lockup:
	$(eval TMP=$(shell make -s -C ../lockup deploy))
	$(eval TEST_LOCKUP_PROGRAM_ID=$(shell echo $(TMP) | sed 's/.*{programId: \(.*\)}.*/\1/g'))

test-program:
	make \
	TEST_PROGRAM_ID=$(TEST_PROGRAM_ID) \
	TEST_LOCKUP_PROGRAM_ID=$(TEST_LOCKUP_PROGRAM_ID) \
	TEST_STAKE_PROGRAM_ID=$(TEST_STAKE_PROGRAM_ID) \
	test-program-super

deploy-all: deploy-stake self-deploy
	echo '{ "registryProgramId": "$(TEST_PROGRAM_ID)", "stakeProgramId": "$(TEST_STAKE_PROGRAM_ID)", "lockupProgramId": "$(TEST_LOCKUP_PROGRAM_ID)" }'
