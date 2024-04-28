# Based on https://github.com/robotframework/PythonRemoteServer/blob/master/example/tests.robot
#
# run with customised, local robotframework
#
#    python3 ../robotframework/src/robot/run.py -L TRACE -d Reports tests/PythonRemoteServer_example/tests.robot
#

*** Settings ***
Library       Remote    http://${ADDRESS}:${PORT}

*** Variables ***
${ADDRESS}    127.0.0.1
${PORT}       8270

*** Test Cases ***
Count Items in Directory
    ${items1} =    Count Items In Directory    ${CURDIR}
    ${items2} =    Count Items In Directory    ${TEMPDIR}
    Log    ${items1} items in '${CURDIR}' and ${items2} items in '${TEMPDIR}'

Failing Example
    Strings Should Be Equal    Hello    Hello
    #Strings Should Be Equal    not      equal
    Run Keyword and Expect Error    Given strings are not equal.    Strings Should Be Equal    not      equal

Adder
    ${result} =    Addone    ${42}
    Should Be Equal    ${result}    ${43}

Adder Fails
    Run Keyword and Expect Error    Type mismatch:*    Addone    Nope
    Run Keyword and Expect Error    Can't parse parameter*    Addone

    # TODO Does not match?
    # Run Keyword and Expect Error    Can't parse parameter [1, 1]    Addone    ${1}    ${1}
    Run Keyword and Expect Error    Can't parse parameter*    Addone    ${1}    ${1}

