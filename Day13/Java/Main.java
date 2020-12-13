import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

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

		int departing = 0;
		List<Integer> busIds = new ArrayList<>();
		Map<Integer, Integer> busses = new HashMap<>();
		int fastest = 0;
		for (String line : Files.readAllLines(input.toPath())) {
			if (departing == 0) {
				departing = Integer.parseInt(line);
			} else {
				for (String bus : line.split(",")) {
					if (!bus.equals("x")) {
						int b = Integer.parseInt(bus);
						busses.put(b, b - departing % b);
						if (fastest == 0 || busses.get(b) < busses.get(fastest)) {
							fastest = b;
						}
						busIds.add(b);
					} else
						busIds.add(0);
				}
			}
		}

		System.out.println("Part one result is " + fastest * busses.get(fastest) + ".");

		long time = 0;
		long step = 1;
		int current = -1;
		while (true) {
			int bus = 0;
			for (int i = current + 1; i < busIds.size(); i++) {
				bus = busIds.get(i);
				if (bus == 0) {
					continue;
				} else if ((time + i) % bus != 0) {
					bus = 0;
					break;
				} else {
					current = i;
					step *= bus;
				}
			}

			if (bus == busIds.get(busIds.size() - 1))
				break;
			else
				time += step;
		}

		System.out.println("Part two timestamp is " + time + ".");
	}
}
