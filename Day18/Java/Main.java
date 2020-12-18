import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Main {

	public static void main(String[] args) throws IOException {
		File input = new File("input").getAbsoluteFile();
		if (!input.exists()) {
			System.out.println(input.getPath() + " does not exist!");
			System.out.println("trying " + input.getPath() + ".txt instead.");
			input = new File(input.getPath() + ".txt");
			if (!input.exists()) {
				System.err.println("neither " + input.getPath().substring(0, input.getPath().length() - 4) + ",");
				System.err.println("nor " + input.getPath() + " exists!");
				return;
			}
		}

		long globalResult = 0;
		long globalResult2 = 0;
		for (String line : Files.readAllLines(input.toPath())) {
			globalResult += evaluate(line);
			globalResult2 += evaluate2(line);
		}

		System.out.println("The result of all equation results for part 1 added together is " + globalResult + ".");
		System.out.println("The result of all equation results for part 2 added together is " + globalResult2 + ".");
	}

	private static long evaluate(String equation) {
		long result = 0;
		char operation = '+';
		int i = 0;
		while (i < equation.length()) {
			char c = equation.charAt(i);
			if (c == ' ') {
				i++;
				continue;
			} else if (c == '(') {
				int open = 0;
				int end = 0;
				for (int j = i; j < equation.length(); j++) {
					if (equation.charAt(j) == '(') {
						open++;
					} else if (equation.charAt(j) == ')') {
						open--;
					}

					if (open == 0) {
						end = j;
						break;
					}
				}

				if (operation == '+') {
					result += evaluate(equation.substring(i + 1, end));
				} else if (operation == '*') {
					result *= evaluate(equation.substring(i + 1, end));
				}
				i = end;
			} else if (Character.isDigit(c)) {
				if (operation == '+') {
					result += Integer.parseInt(String.valueOf(c));
				} else if (operation == '*') {
					result *= Integer.parseInt(String.valueOf(c));
				}
			} else {
				operation = c;
			}
			i++;
		}

		return result;
	}

	private static long evaluate2(String equation) {
		int open = 0;
		while (equation.contains("(")) {
			int start = equation.indexOf('(');
			int end = 0;
			for (int i = start; i < equation.length(); i++) {
				if (equation.charAt(i) == '(') {
					open++;
				} else if (equation.charAt(i) == ')') {
					open--;
				}

				if (open == 0) {
					end = i;
					break;
				}
			}

			equation = equation.replaceFirst(
					equation.substring(start, end + 1).replaceAll("\\+", "\\\\+").replaceAll("\\*", "\\\\*")
							.replaceAll("\\(", "\\\\(").replaceAll("\\)", "\\\\)"),
					String.valueOf(evaluate2(equation.substring(start + 1, end))));
		}

		Pattern nr = Pattern.compile("\\d+");
		while (equation.contains("+")) {
			int index = equation.indexOf('+');
			Matcher first = nr.matcher(equation.substring(Math.max(0, equation.lastIndexOf(' ', index - 2)), index));
			first.find();
			Matcher second = nr.matcher(equation.substring(index));
			second.find();
			equation = equation.replaceFirst("[0-9]+ \\+ [0-9]+",
					String.valueOf(Long.parseLong(first.group()) + Long.parseLong(second.group())));
		}

		while (equation.contains("*")) {
			int index = equation.indexOf('*');
			Matcher first = nr.matcher(equation.substring(Math.max(0, equation.lastIndexOf(' ', index - 2)), index));
			first.find();
			Matcher second = nr.matcher(equation.substring(index));
			second.find();
			equation = equation.replaceFirst("[0-9]+ \\* [0-9]+",
					String.valueOf(Long.parseLong(first.group()) * Long.parseLong(second.group())));
		}

		return Long.parseLong(equation);
	}
}
