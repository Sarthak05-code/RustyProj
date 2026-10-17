def notyet():
    minus = 2023 - int(age)
    print(f"Your BirthDate is: {minus} ")


def yesyet():
    plus = Current - int(age)
    print(f"Your BirthDate is: {plus}")


def one():
    if confirm == "y":
        print(f"Your complete BirthDate and Day is: {month}", "and", Current - int(age))
    elif confirm == "n":
        print(f"Your complete BirthDate and Day is: {month}", "and", 2023 - int(age))
    else:
        print("Error")


def two():
    if confirm == "y":
        print(f"Your complete BirthDate and Day is: {month}", "and", Current - int(age))
    elif confirm == "n":
        print(f"Your complete BirthDate and Day is: {month}", "and", 2023 - int(age))
    else:
        print("Error")


def three():
    if confirm == "y":
        print(f"Your complete BirthDate and Day is: {month}", "and", Current - int(age))
    elif confirm == "n":
        print(f"Your complete BirthDate and Day is: {month}", "and", 2023 - int(age))
    else:
        print("Error")


def four():
    if confirm == "y":
        print(f"Your complete BirthDate and Day is: {month}", "and", Current - int(age))
    elif confirm == "n":
        print(f"Your complete BirthDate and Day is: {month}", "and", 2023 - int(age))
    else:
        print("Error")


def five():
    if confirm == "y":
        print(f"Your complete BirthDate and Day is: {month}", "and", Current - int(age))
    elif confirm == "n":
        print(f"Your complete BirthDate and Day is: {month}", "and", 2023 - int(age))
    else:
        print("Error")


def six():
    if confirm == "y":
        print(f"Your complete BirthDate and Day is: {month}", "and", Current - int(age))
    elif confirm == "n":
        print(f"Your complete BirthDate and Day is: {month}", "and", 2023 - int(age))
    else:
        print("Error")


def seven():
    if confirm == "y":
        print(f"Your complete BirthDate and Day is: {month}", "and", Current - int(age))
    elif confirm == "n":
        print(f"Your complete BirthDate and Day is: {month}", "and", 2023 - int(age))
    else:
        print("Error")


def eight():
    if confirm == "y":
        print(f"Your complete BirthDate and Day is: {month}", "and", Current - int(age))
    elif confirm == "n":
        print(f"Your complete BirthDate and Day is: {month}", "and", 2023 - int(age))
    else:
        print("Error")


def nine():
    if confirm == "y":
        print(f"Your complete BirthDate and Day is: {month}", "and", Current - int(age))
    elif confirm == "n":
        print(f"Your complete BirthDate and Day is: {month}", "and", 2023 - int(age))
    else:
        print("Error")


def ten():
    if confirm == "y":
        print(f"Your complete BirthDate and Day is: {month}", "and", Current - int(age))
    elif confirm == "n":
        print(f"Your complete BirthDate and Day is: {month}", "and", 2023 - int(age))
    else:
        print("Error")


def eleven():
    if confirm == "y":
        print(f"Your complete BirthDate and Day is: {month}", "and", Current - int(age))
    elif confirm == "n":
        print(f"Your complete BirthDate and Day is: {month}", "and", 2023 - int(age))
    else:
        print("Error")


def twelve():
    if confirm == "y":
        print(f"Your complete BirthDate and Day is: {month}", "and", Current - int(age))
    elif confirm == "n":
        print(f"Your complete BirthDate and Day is: {month}", "and", 2023 - int(age))
    else:
        print("Error")


def ones():
    print(f"Your Months and Age is: {year} in leap years and on the month,{months}")


def twos():
    print(f"Your Months and Age is: {year} in leap years and on the month,{months}")


def threes():
    print(f"Your Months and Age is: {year} in leap years and on the month,{months}")


def fours():
    print(f"Your Months and Age is: {year} in leap years and on the month,{months}")


def fives():
    print(f"Your Months and Age is: {year} in leap years and on the month,{months}")


def sixs():
    print(f"Your Months and Age is: {year} in leap years and on the month,{months}")


def sevens():
    print(f"Your Months and Age is: {year} in leap years and on the month,{months}")


def eights():
    print(f"Your Months and Age is: {year} in leap years and on the month,{months}")


def nines():
    print(f"Your Months and Age is: {year} in leap years and on the month,{months}")


def tens():
    print(f"Your Months and Age is: {year} in leap years and on the month,{months}")


def elevens():
    print(f"Your Months and Age is: {year} in leap years and on the month,{months}")


def twelves():
    print(f"Your Months and Age is: {year} in leap years and on the month,{months}")


def complete():
    if confirm == "y":
        print(f"Your complete D.O.B is:{day} {month}", Current - int(age))
    elif confirm == "n":
        print(f"Your complete D.O.B is:{day} {month}", 2023 - int(age))
    else:
        print("Error")


def covert():
    cals = float(year) * lap
    print(f"Your Age Is : {cals} in non-leap year")


def leapers():
    cal = float(year) * lap
    print(f"Your Actual Age is:{date} {months} {cal}years old {Current - cal}")
    


lap = 4
Current = 2024

print("-------------------------")
print("BirthDate Check Simulator")
print("Enter 'E/e' If You Wanna Exit")
print("-------------------------")

print("First of all , Are Your A leaping?: ")
print("Press Any  Other Alphabet Key Than Y to exit This Query")
leaper = input("Y/N: ").upper()
if leaper == "Y":
    year = float(input("Your Age In Leap Year: "))
    covert()
    print("Your BirthMonths?: ")
    Option = (
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    )
    man = None
    while man not in Option:
        months = input(" January~December: ")
        if months == "January":
            ones()
            break
        elif months == "February":
            twos()
            break

        elif months == "March":
            threes()
            break
        elif months == "April":
            fours()
            break
        elif months == "May":
            fives()
            break
        elif months == "June":
            sixs()
            break
        elif months == "July":
            sevens()
            break
        elif months == "August":
            eights()
            break
        elif months == "September":
            nines()
            break
        elif months == "October":
            tens()
            break
        elif months == "November":
            elevens()
            break
        elif months == "December":
            twelves()
            break
    else:
        print("/++++++/")
        print("Error")
        print("/++++++/")
    option2 = (
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "10",
        "11",
        "12",
        "13",
        "14",
        "15",
        "16",
        "17",
        "18",
        "19",
        "20",
        "21",
        "22",
        "23",
        "24",
        "25",
        "26",
        "27",
        "28",
        "29",
        "30",
        "31",
        "32",
    )
    lal = None
    while lal not in option2:
        print("Now Your Day Of Birth")
        date = input("Your Day of Birth(1~32): ")
        if "1" <= date <= "32":
            leapers()
            break
        else:
            print("Error")


else:
    age = input("Enter Your Age: ")
    if age == "E" or age == "e":
        print("Thank You For Your Time")

    elif age.isalpha():
        print("//////")
        print(" Error")
        print("//////")
    else:
        option = ("y", "n")
        calm = None
        while calm not in option:
            confirm = input("Has Your Birthday been Celebrated Yet?: ")
            if confirm == "Y".lower():
                yesyet()
                break
            elif confirm == "N".lower():
                notyet()
                break
            else:
                print("/++++++/")
                print("Error")
                print("/++++++/")

        if age == "E" and age == "e":
            print("End")
        else:
            print("Your BirthMonths?: ")
            Option = (
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            )
            man = None
            while man not in Option:
                month = input(" January~December: ")
                if month == "January":
                    one()
                    break
                elif month == "February":
                    two()
                    break

                elif month == "March":
                    three()
                    break
                elif month == "April":
                    four()
                    break
                elif month == "May":
                    five()
                    break
                elif month == "June":
                    six()
                    break
                elif month == "July":
                    seven()
                    break
                elif month == "August":
                    eight()
                    break
                elif month == "September":
                    nine()
                    break
                elif month == "October":
                    ten()
                    break
                elif month == "November":
                    eleven()
                    break
                elif month == "December":
                    twelve()
                    break
            else:
                print("/++++++/")
                print("Error")
                print("/++++++/")

        if age == "E" and age == "e":
            print("End")
        else:
            option2 = (
                "1",
                "2",
                "3",
                "4",
                "5",
                "6",
                "7",
                "8",
                "9",
                "10",
                "11",
                "12",
                "13",
                "14",
                "15",
                "16",
                "17",
                "18",
                "19",
                "20",
                "21",
                "22",
                "23",
                "24",
                "25",
                "26",
                "27",
                "28",
                "29",
                "30",
                "31",
                "32",
            )
            balm = None
            while balm not in option2:
                print("Now Your Day Of Birth")
                day = input("Your Day of Birth: ")
                if "1" <= day <= "32":
                    complete()
                    break
                else:
                    print("Error")
