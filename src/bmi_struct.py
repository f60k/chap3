height_cm = float(input("HEIGHT?(cm)"))
weight = float(input("WEIGHT(kg)"))

height = height_cm / 100.0
bmi = weight / (height**2)

bmi_list = [
    {"min": 0, "max": 18.5, "label": "LOW"},
    {"min": 18.5, "max": 25, "label": "NORMAL"},
    {"min": 25, "max": 30, "label": "FAT1"},
    {"min": 30, "max": 35, "label": "FAT2"},
    {"min": 35, "max": 40, "label": "FAT3"},
    {"min": 40, "max": 99, "label": "FAT4"},
]

result = "UNKNOWN"
for range in bmi_list:
    if range["min"] <= bmi < range["max"]:
        result = range["label"]

print("BMI={:.1f}, RESULT={}".format(bmi, result))
