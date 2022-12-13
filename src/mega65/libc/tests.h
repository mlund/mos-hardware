#ifndef TESTS_H
#define TESTS_H

#define TEST_START 0xf0
#define TEST_SKIP 0xf1
#define TEST_PASS 0xf2
#define TEST_FAIL 0xf3
#define TEST_ERROR 0xf4
#define TEST_LOG 0xfd
#define TEST_SETNAME 0xfe
#define TEST_DONEALL 0xff

// convenience methods

/**
 * \brief Setup of the unit test reporting to the host machine
 * \param param testName Human readable name of the test
 * \param issueNum The mega65-core issue number that identifies the test issue
 */
void unit_test_setup(char* testName, unsigned short issueNum);

/**
 * \brief Report a successful test with a optional message
 * \param msg Description of the successful test (NULL uses global testName)
 */
void unit_test_ok(char* msg);

/**
 * \brief Report a failed test with a optional message
 * \param msg Description of the failed test (NULL uses global testName)
 */
void unit_test_fail(char* msg);

/**
 * \brief Finish test procedure and tell m65 to exit
 */
void unit_test_done(void);

// low level functions

/**
 * \brief Reports unit test result to the host machine
 * \param issue The issue number that identifies the test issue
 * \param sub The sub issue number (for multiple tests per issue)
 * \param status The test status to be sent
 */
void unit_test_report(unsigned short issue, unsigned char sub, unsigned char status);

/**
 * \brief Reports current test name to the host machine
 * \param name The human-readable name of the current test
 */
void unit_test_set_current_name(char* name);

/**
 * \brief Logs a message on the host machine
 * \param msg The message to be logged
 */
void unit_test_log(char* msg);

#endif
