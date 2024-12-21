CC = gcc
CFLAGS = -fPIC -Wall -Wextra -O2
LDFLAGS = -shared
TARGET = libssd1306.so

SRC = main.c linux_i2c.c ssd1306.c
HEADERS = linux_i2c.h ssd1306.h font.h
OBJS = $(SRC:.c=.o)

all: $(TARGET)

$(TARGET): $(OBJS)
	$(CC) $(LDFLAGS) -o $@ $(OBJS)

%.o: %.c $(HEADERS)
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJS) $(TARGET)

