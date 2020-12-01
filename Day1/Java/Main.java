import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.List;

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

		List<Integer> inputs = new ArrayList<Integer>();
		boolean Double = false;
		boolean Triple = false;
		for(String line:Files.readAllLines(input.toPath())) {
			int nr = Integer.parseInt(line.trim());
			if (!Double && inputs.contains(2020 - nr)) {
				System.out.println("Found matching two number pair!");
				System.out.printf("The numbers are %d and %d, and the result is %d.%n", nr, 2020 - nr, nr * (2020 - nr));
				Double = true;
			}

			if (!Triple) {
				for (int i:inputs) {
					if (inputs.contains(2020 - nr - i)) {
						System.out.println("Found matching three number pair!");
						System.out.printf("The numbers are %d, %d, and %d, and the result is %d.%n", nr, i, 2020 - nr - i, nr * i * (2020 - nr - i));
						Triple = true;
						break;
					}
				}
			}

			inputs.add(nr);
			if (Double && Triple) {
				return;
			}
		}

		System.err.println("Could not find a matching number pair!");
	}
	
}
