#!/usr/bin/env ruby
code = open(ARGV[0]).read.chars
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
    if code[ip] == '>'
        dp += 1
    elsif code[ip] == '<'
        dp -= 1
    elsif code[ip] == '+'
        data[dp] = (data[dp].ord + 1).chr
    elsif code[ip] == '-'
        data[dp] = (data[dp].ord - 1).chr
    elsif code[ip] == '.'
        print data[dp]
    elsif code[ip] == ','
        data[dp] = gets
    elsif code[ip] == '['
        if data[dp].ord == 0
            ip = loop_map[ip]
        end
    elsif code[ip] == ']'
        if data[dp].ord != 0
            ip = loop_map[ip]
        end
    end
    ip += 1
end
