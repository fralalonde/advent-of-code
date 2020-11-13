program Project1;

const
  target: longint = 368078;

var
  x: longint = 0;
  y: longint = 0;

  procedure SpiralOut;
  var
    idx: longint = 1;
    ring: longint = 0;
    move: longint;
  begin
    //while idx <> 368078 do
    repeat
      move := ring * 2 + 1;
      // right
      Inc(x, move);
      Inc(idx, move);
      WriteLn('x ', x, ' y ', y, ' Idx ', idx, ' ring ', ring);
      if idx >= target then
      begin
         Dec(x, idx - target);
         Exit
      end;

      // up
      Inc(y, move);
      Inc(idx, move);
      Inc(Ring);
      WriteLn('x ', x, ' y ', y, ' Idx ', idx, ' ring ', ring);
      if idx >= target then
      begin
         Dec(y, idx - target);
         Exit
      end;

      // left
      move := ring * 2;
      Dec(x, move);
      Inc(idx, move);
      WriteLn('x ', x, ' y ', y, ' Idx ', idx, ' ring ', ring);
      if idx >= target then
      begin
         Inc(x, idx - target);
         Exit
      end;

      // down
      Dec(y, move);
      Inc(idx, move);
      WriteLn('x ', x, ' y ', y, ' Idx ', idx, ' ring ', ring);
      if idx >= target then
      begin
         Inc(y, idx - target);
         Exit
      end;
     until false;
  end;

begin
  SpiralOut;
  WriteLn('x ', x, ' y ', y, ' target ', target);
  WriteLn(abs(x) + abs(y));
end.
