// THIS CODE SUCKS, should use cache of
// last result with ring-proportional match fn

program Project2;

const
  OFFSET = 500;
  target: longint = 368078;

type
   darray = array of array of integer;

var
  a: darray;
  s, t: longint;

  procedure PrintIt;
  begin
    Write(#9);
    for t := 495 to 505 do
    begin
      Write(t - OFFSET, #9);
    end;
    WriteLn;

    for s := 505 downto 495 do
    begin
      Write(s - OFFSET, #9);
      for t := 495 to 505 do
      begin
        Write(a[t, s], #9);
      end;
      WriteLn;
    end;
  end;

  procedure PrintItAndWait;
  begin
    PrintIt;
    ReadLn;
  end;

  function SpiralOut: longint;
  var
    x: longint = OFFSET;
    y: longint = OFFSET;
    side: longint = -1;
    last: longint;
    i, ref: longint;

  begin
    a[x,y] := 1;
    last := a[x,y];
    PrintItAndWait;

    repeat
      Inc(side);
      ref := y + 1;
      // move RIGHT
      for i := 0 to side do
      begin
        Inc(x);
        Inc(last, a[x - 1, ref]);
        Inc(last, a[x, ref]);
        Inc(last, a[x + 1, ref]);
        if last >= target then Exit(last);
        a[x, y] := last;
        PrintItAndWait;
      end;
      WriteLn('right ', last);

      // move UP
      ref := x - 1;
      for i := 0 to side do
      begin
        Inc(y);
        Inc(last, a[ref, y]);
        Inc(last, a[ref, y + 1]);
        Inc(last, a[ref, y - 1]);
        if last >= target then Exit(last);
        a[x, y] := last;
        WriteLn('up    ', last);
        PrintItAndWait;
      end;

      Inc(side);
      ref := y - 1;
      // move LEFT
      for i := 0 to side do
      begin
        Dec(x);
        Inc(last, a[x - 1, ref]);
        Inc(last, a[x, ref]);
        Inc(last, a[x + 1, ref]);
        if last >= target then Exit(last);
        a[x, y] := last;
        WriteLn('left  ', last);
      end;

      // move DOWN
      ref := x + 1;
      for i := 0 to side do
      begin
        Dec(y);
        Inc(last, a[ref, y + 1]);
        Inc(last, a[ref, y]);
        Inc(last, a[ref, y - 1]);
        if last >= target then Exit(last);
        a[x, y] := last;
        WriteLn('down  ', last);
      end;
    until false;
  end;

begin
  // ah, fuck it
  setlength(a, 10000, 10000);

  WriteLn(SpiralOut);

  PrintIt;
end.
