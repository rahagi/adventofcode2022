CURR_DAY=$(shell ls src | grep day | wc -l)
NEW_DAY=$(shell echo $$(($(CURR_DAY) + 1)))

newday:
	bash ./bin/new-day.sh 0$(NEW_DAY)
	bash ./bin/get-input.sh 0$(NEW_DAY)
