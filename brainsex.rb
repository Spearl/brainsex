#!/usr/bin/env ruby
valid = ['.', ',', '[', ']', '<', '>', '+', '-']
code = open(ARGV[0]).each_char.select { |c| valid.member? c }

loop_stack, loop_map = [], {}
code.each_with_index do |c, i|
  if c == '['
    loop_stack.push i
  elsif c == ']'
    entrance = loop_stack.pop
    loop_map[entrance], loop_map[i] = i, entrance
  end
end

data = ("\x00" * 30000).chars
ip = dp = 0
while ip < code.length && ip >= 0
  case code[ip]
    when '>'
      dp += 1
    when '<'
      dp -= 1
    when '+'
      data[dp] = (data[dp].ord + 1).chr
    when '-'
      data[dp] = (data[dp].ord - 1).chr
    when '.'
      print data[dp]
    when ','
      data[dp] = gets
    when '['
      ip = loop_map[ip] if data[dp].ord == 0
    when ']'
      ip = loop_map[ip] if data[dp].ord != 0
  end
  ip += 1
end
