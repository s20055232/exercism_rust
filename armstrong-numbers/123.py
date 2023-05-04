def no_of_digits(num):
    i = 0
    while num > 0:
        num //= 10
        i += 1

    return i


def required_sum(num):
    i = no_of_digits(num)
    s = 0

    while num > 0:
        digit = num % 10
        num //= 10
        s += pow(digit, i)

    return s


num = int(input("Enter number:"))
s = required_sum(num)

if s == num:
    print("Armstrong Number")
else:
    print("Not Armstrong Number")
