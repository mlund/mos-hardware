#include <stdint.h>
#include "memory.h"

volatile unsigned char __tests_out;
volatile unsigned short __ut_issueNum;
volatile unsigned char __ut_subissue;

void unit_test_report(unsigned short issue, unsigned char sub, unsigned char status)
{
  __tests_out = issue & 0xff;
  asm volatile("LDA __tests_out\n"
               "STA $D643\n"
               "NOP" ::: "a");
  __tests_out = issue >> 8;
  asm volatile("LDA __tests_out\n"
               "STA $D643\n"
               "NOP" ::: "a");
  __tests_out = sub;
  asm volatile("LDA __tests_out\n"
               "STA $D643\n"
               "NOP" ::: "a");
  __tests_out = status;
  asm volatile("LDA __tests_out\n"
               "STA $D643\n"
               "NOP" ::: "a");
}

void _unit_test_msg(char* msg, char cmd)
{
  unsigned char* current;

  unit_test_report(0, 0, cmd);
  current = msg;

  while (*current) {
    __tests_out = *current++;
    asm volatile("LDA __tests_out\n"
                 "STA $D643\n"
                 "NOP" ::: "a");
  }

  asm volatile("LDA #92\n"
               "STA $D643\n"
               "NOP" ::: "a");
}

void unit_test_set_current_name(char* name)
{
  _unit_test_msg(name, 0xfe);
}

void unit_test_log(char* msg)
{
  _unit_test_msg(msg, 0xfd);
}

void unit_test_setup(char* testName, unsigned short issueNum)
{
  POKE(0xD02F, 0x47);
  POKE(0xD02F, 0x53);
  __ut_issueNum = issueNum;
  __ut_subissue = 0;
  unit_test_set_current_name(testName);
  unit_test_report(__ut_issueNum, __ut_subissue, 0xf0);
}

void unit_test_ok(char* msg)
{
  if (msg) {
    unit_test_log(msg);
  }
  unit_test_report(__ut_issueNum, __ut_subissue, 0xf2);
  ++__ut_subissue;
}

void unit_test_fail(char* msg)
{
  if (msg) {
    unit_test_log(msg);
  }
  unit_test_report(__ut_issueNum, __ut_subissue, 0xf3);
  ++__ut_subissue;
}

void unit_test_done(void)
{
  unit_test_report(__ut_issueNum, __ut_subissue, 0xff);
}
