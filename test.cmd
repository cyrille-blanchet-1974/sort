@echo off
call build.cmd

set prg=.\target\debug\sort.exe

echo a>test.fic
echo c>>test.fic
echo b>>test.fic
echo d>>test.fic
%prg%  test.fic
echo expected: a b c d 
pause

%prg% -r test.fic
echo expected: d c b a 
pause

