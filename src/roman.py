x = int(input("Digite um número\n"))

lista_romanas = {
    "Unidade":["I","II","III","IV","V","VI","VII","VIII","IX","X"],
    "Dezena":["X","XX","XXX","XL","L","LX","LXX","LXX","XC","C"],
    "Centena":["C","CC","CCC","CD","D","DC","DCC","DCCC","CM","M"],
    "Milhar":["M","MM","MMM"]
}

lista_romana = []
cont = 1 
while True:
    if cont == 1: 
        resultado = lista_romanas["Unidade"][int((x%10)-1)]
        lista_romana.append(resultado)
    elif cont == 2:
        resultado = lista_romanas["Dezena"][int((x%10)-1)]
        lista_romana.append(resultado)
    elif cont == 3:
        resultado = lista_romanas["Centena"][int((x%10)-1)]
        lista_romana.append(resultado)
    elif cont == 4:
        resultado = lista_romanas["Milhar"][int((x%10)-1)]
        lista_romana.append(resultado)

    if x < 10:
        break
    x = x/10
    cont += 1

romano = "".join(lista_romana)
print(romano[::-1])